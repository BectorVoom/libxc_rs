//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 989/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk989(t3177: f64, t3436: f64, t1194: f64, t381: f64, t1095: f64, t1169: f64, t983: f64, t9538: f64, t3621: f64, t426: f64, t187: f64, t2997: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10745 = t3177 * t3436;
    let t10752 = t381 * t1194;
    let t10753 = t1095 * t10752;
    let t10787 = t1169 * t983;
    let t10799 = t9538 * t381;
    let t10819 = 1.0_f64 / t3621 / t426;
    let t10845 = t187 * t2997;
    (t10745, t10752, t10753, t10787, t10799, t10819, t10845)
}
