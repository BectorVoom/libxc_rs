//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1829/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1829(t10255: f64, t4531: f64, t343: f64, t4540: f64, t984: f64, t4546: f64, t12606: f64, t978: f64, t977: f64, t135: f64, t340: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13806 = t4531 * t10255;
    let t13812 = t4540 * t984 * t343;
    let t13813 = t4546 * t13812;
    let t13816 = t978 * t12606;
    let t13817 = t977 * t13816;
    let t13822 = t135 * t340;
    (t13806, t13812, t13813, t13816, t13817, t13822)
}
