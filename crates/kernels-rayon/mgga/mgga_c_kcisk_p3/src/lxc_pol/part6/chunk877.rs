//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 877/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk877(t1835: f64, t28381: f64, t1842: f64, t1856: f64, t1659: f64, t28373: f64, t28389: f64, t2063: f64, t7718: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28645 = t1835 * t28381;
    let t28648 = t1842 * t28381;
    let t28651 = t1856 * t28381;
    let t28654 = t1659 * t28373;
    let t28657 = t1835 * t28373;
    let t28660 = t1842 * t28389;
    let t28663 = t2063 * t7718;
    (t28645, t28648, t28651, t28654, t28657, t28660, t28663)
}
