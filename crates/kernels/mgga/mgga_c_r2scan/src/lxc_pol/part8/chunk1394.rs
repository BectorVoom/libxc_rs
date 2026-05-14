//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1394/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1394<F: Float>(t2060: F, t2062: F, t9903: F, t7902: F, t9033: F, t19712: F, t20180: F, t22677: F, t25041: F, t32998: F, t32999: F, t33000: F, t33001: F, t33002: F, t33003: F, t11: F, t146: F, t147: F, t21036: F, t21038: F, t24973: F, t24996: F, t25001: F, t25182: F, t25189: F, t25210: F, t26356: F, t26360: F, t279: F, t28460: F, t28463: F, t29552: F, t29561: F, t29568: F, t29599: F, t29604: F, t29613: F, t33395: F, t33405: F, t33414: F, t33423: F, t33433: F, t33439: F, t33447: F, t33459: F, t33466: F, t33474: F, t33483: F, t33490: F, t33513: F, t33525: F, t33534: F, t33544: F, t33550: F, t33564: F, t33583: F, t33590: F, t33594: F, t33608: F, t33620: F, t33630: F, t33638: F, t33646: F, t33653: F, t33662: F, t33677: F, t33686: F, t33693: F, t33700: F, t33704: F, t33709: F, t33722: F, t33724: F, t33731: F, t33733: F, t33739: F, t33752: F, t33756: F, t33762: F, t33770: F, t33773: F, t33776: F, t33780: F, t33783: F, t33785: F, t33790: F, t5: F) -> (F,) {
    let t33792 = t2060 * t9903 * t2062;
    let t33794 = t9033 * t7902;
    let t33796 = t22677 - t32998 - t25041 + t19712 + t32999 + t20180 + t33000 + t33001 + t33002 - t33003 - 0.675260332e-1 * t33792 - 0.2025780996e0 * t33794;
    let t33812 = 0.38415120233790484326e0 * t33395 - t24973 - 0.13869154784086829701e1 * t29552 - t24996 - t25001 - 0.25426783770825854452e1 * t29561 + 0.25426783770825854452e1 * t29568 + 0.43341108700271342816e-1 * t146 * t147 * (t21036 - 140.0 / 27.0 * t21038 - 140.0 / 9.0 * t26356 + t26360 + 20.0 / 3.0 * t28460 - 5.0 * t28463 - 5.0 / 3.0 * t33405 + 5.0 * t5 * t11 * (t33620 + t33525 + t33646 + t33433 + t33583 + t33709 + t33490 + t33414 + t33590 + t33466 + t33700 + t33459 + t33608 + t33564 + t33662 + t33704 + t33630 + t33550 + t33686 + t33653 + t33513 + t33534 + t33483 + t33594 + t33544 + t33677 + t33423 + t33693 + t33447 + t33474 + t33638 + t33439) - 45.0 * param_eta * (t33722 + t33724 + t33731 + t33733 + t33739 + t33752 + t33756 + t33762 + t33770 + t33773 + t33776 + t33780 + t33783 + t33785 + t33790 + t33796)) * t279 - 0.34930954652346593433e-1 * t29599 + t25182 + 0.12225834128321307702e1 * t29604 + t25189 - 0.22084125774650235182e1 * t25210 - 0.17465477326173296717e-1 * t29613;
    (t33812,)
}
