//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1630/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1630<F: Float>(t11262: F, t3711: F, t3713: F, t3584: F, t3588: F, t1250: F, t12787: F, t12803: F, t12810: F, t12840: F, t12847: F, t13102: F, t17429: F, t17638: F, t17644: F, t17688: F, t17709: F, t17747: F, t17753: F, t3625: F, t3626: F, t3629: F, t3647: F, t3718: F, t3720: F, t44501: F, t44711: F, t44726: F, t44729: F, t44738: F, t44748: F, t5331: F) -> (F, F) {
    let t44751 = t3711 * t11262 * t3713;
    let t44753 = t3584 * t3588;
    let t44758 = -F::cast_from(0.2540682555144873302e-2_f64) * t3647 * t13102 + F::cast_from(0.11433071498151929859e-2_f64) * t44711 + F::cast_from(0.14291339372689912324e-2_f64) * t3625 * t12787 * t12803 * t17688 - F::cast_from(0.57165357490759649296e-3_f64) * t17753 * t3626 * t44501 * t3629 + F::cast_from(0.17149607247227894789e-2_f64) * t5331 * t3626 * t12810 * t17644 - F::cast_from(0.11433071498151929859e-2_f64) * t44726 - F::cast_from(0.22866142996303859718e-2_f64) * t44729 + F::cast_from(0.17149607247227894789e-2_f64) * t17429 * t12847 + F::cast_from(0.85748036236139473944e-3_f64) * t5331 * t3626 * t12810 * t17638 - F::cast_from(0.34299214494455789578e-2_f64) * t17709 * t3626 * t44501 * t44738 + F::cast_from(0.34299214494455789578e-2_f64) * t17747 * t3626 * t44501 * t12840 - F::cast_from(0.34299214494455789578e-2_f64) * t44748 - F::cast_from(0.3811023832717309953e-3_f64) * t44751 - F::cast_from(0.12862205435420921092e-2_f64) * t3718 * t3720 * t44753 * t1250;
    (t44753, t44758)
}
