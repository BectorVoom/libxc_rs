//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1630/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1630(t11262: f64, t3711: f64, t3713: f64, t3584: f64, t3588: f64, t1250: f64, t12787: f64, t12803: f64, t12810: f64, t12840: f64, t12847: f64, t13102: f64, t17429: f64, t17638: f64, t17644: f64, t17688: f64, t17709: f64, t17747: f64, t17753: f64, t3625: f64, t3626: f64, t3629: f64, t3647: f64, t3718: f64, t3720: f64, t44501: f64, t44711: f64, t44726: f64, t44729: f64, t44738: f64, t44748: f64, t5331: f64) -> (f64, f64) {
    let t44751 = t3711 * t11262 * t3713;
    let t44753 = t3584 * t3588;
    let t44758 = -0.2540682555144873302e-2_f64 * t3647 * t13102 + 0.11433071498151929859e-2_f64 * t44711 + 0.14291339372689912324e-2_f64 * t3625 * t12787 * t12803 * t17688 - 0.57165357490759649296e-3_f64 * t17753 * t3626 * t44501 * t3629 + 0.17149607247227894789e-2_f64 * t5331 * t3626 * t12810 * t17644 - 0.11433071498151929859e-2_f64 * t44726 - 0.22866142996303859718e-2_f64 * t44729 + 0.17149607247227894789e-2_f64 * t17429 * t12847 + 0.85748036236139473944e-3_f64 * t5331 * t3626 * t12810 * t17638 - 0.34299214494455789578e-2_f64 * t17709 * t3626 * t44501 * t44738 + 0.34299214494455789578e-2_f64 * t17747 * t3626 * t44501 * t12840 - 0.34299214494455789578e-2_f64 * t44748 - 0.3811023832717309953e-3_f64 * t44751 - 0.12862205435420921092e-2_f64 * t3718 * t3720 * t44753 * t1250;
    (t44753, t44758)
}
