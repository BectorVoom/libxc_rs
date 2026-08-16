//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2453/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2453(t17635: f64, t4337: f64, t10254: f64, t21510: f64, t13769: f64, t13835: f64, t13839: f64, t1409: f64, t17748: f64, t17800: f64, t17804: f64, t2986: f64, t2988: f64, t2989: f64, t4518: f64, t4531: f64, t4540: f64, t5681: f64, t5685: f64, t61082: f64, t61103: f64, t61279: f64, t61307: f64, t61310: f64, t61313: f64, t61322: f64, t61327: f64, t61365: f64, t6733: f64) -> (f64, f64) {
    let t69742 = t4337 * t17635;
    let t69746 = t10254 * t21510;
    let t69791 = 0.49999999999999999998e-2_f64 * t2986 * t4518 * t69742 + 0.16666666666666666666e-2_f64 * t2986 * t2988 * t69746 + 0.16666666666666666666e-2_f64 * t2986 * t17800 * t13835 - 0.11111111111111111111e-2_f64 * t2986 * t61322 * t13839 + 0.22222222222222222222e-2_f64 * t61307 + 0.55555555555555555554e-3_f64 * t61310 + 0.55555555555555555554e-3_f64 * t61313 - 0.27777777777777777777e-3_f64 * t61327 - 0.49999999999999999998e-2_f64 * t2986 * t4531 * t61082 - 0.11111111111111111111e-2_f64 * t2986 * t13769 * t61279 - 0.83333333333333333331e-3_f64 * t2986 * t4531 * t6733 * t5685 + 0.33333333333333333332e-2_f64 * t2986 * t4531 * t61103 + 0.16666666666666666666e-2_f64 * t2986 * t4531 * t6733 * t5681 - 0.16666666666666666666e-2_f64 * t2986 * t4531 * t2989 * t1409 * t4540 + 0.16666666666666666666e-2_f64 * t2986 * t17804 * t13835 - 0.11111111111111111111e-2_f64 * t2986 * t61365 * t13839 - 0.83333333333333333331e-3_f64 * t2986 * t17804 * t17748;
    (t69742, t69791)
}
