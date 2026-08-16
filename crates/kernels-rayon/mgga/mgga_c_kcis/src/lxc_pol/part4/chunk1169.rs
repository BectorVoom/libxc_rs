//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1169/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1169(t14097: f64, t5176: f64, t5175: f64, t14374: f64, t359: f64, t376: f64, t1170: f64, t3474: f64, t5053: f64, t1809: f64, t3448: f64, t10745: f64, t5099: f64) -> (f64, f64, f64, f64, f64) {
    let t14812 = t5176 * t14097;
    let t14813 = t5175 * t14812;
    let t14815 = t359 * t14374;
    let t14816 = t376 * t14815;
    let t14817 = t1170 * t14816;
    let t14819 = t3474 * t5053;
    let t14821 = t1809 * t3448;
    let t14823 = t10745 * t5099;
    (t14813, t14817, t14819, t14821, t14823)
}
