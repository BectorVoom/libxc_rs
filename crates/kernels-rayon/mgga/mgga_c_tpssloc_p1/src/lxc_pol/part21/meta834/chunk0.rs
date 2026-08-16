//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2953/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2953(t12652: f64, t13536: f64, t10236: f64, t17691: f64, t13779: f64, t17183: f64, t2986: f64, t10186: f64, t10235: f64, t13769: f64, t13798: f64, t13839: f64, t13851: f64, t1539: f64, t17748: f64, t17795: f64, t23494: f64, t43055: f64, t4531: f64, t47919: f64, t47927: f64, t47941: f64, t48217: f64, t48221: f64, t48269: f64) -> (f64, f64) {
    let t61524 = t13536 * t12652;
    let t61528 = t10236 * t17691;
    let t61557 = t2986 * t13779 * t17183;
    let t61560 = 0.34567901234567901234e-2_f64 * t2986 * t13798 * t61524 - 0.14814814814814814814e-2_f64 * t2986 * t10235 * t61528 + 0.11111111111111111111e-2_f64 * t2986 * t4531 * t47919 + 0.44444444444444444444e-2_f64 * t2986 * t13769 * t47941 - 0.14814814814814814814e-2_f64 * t2986 * t48217 * t13839 - 0.74074074074074074072e-3_f64 * t2986 * t13769 * t48269 - 0.17283950617283950617e-2_f64 * t2986 * t48221 * t47927 - 0.11111111111111111111e-2_f64 * t2986 * t13851 * t17748 - 0.55555555555555555554e-3_f64 * t2986 * t4531 * t23494 * t1539 + 0.14814814814814814814e-2_f64 * t10186 * t17795 - 0.37037037037037037036e-3_f64 * t61557 + 0.12345679012345679012e-3_f64 * t43055;
    (t61524, t61560)
}
