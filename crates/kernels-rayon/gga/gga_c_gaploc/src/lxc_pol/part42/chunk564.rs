//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 564/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk564(t10827: f64, t969: f64, t825: f64, t2365: f64, t8756: f64, t7390: f64, t3488: f64, t7354: f64, t2684: f64, t8769: f64, t6111: f64, t826: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10828 = t969 * t10827;
    let t10829 = t825 * t10828;
    let t10830 = 0.19171462976960374838e0_f64 * t10829;
    let t10834 = t2365 * t8756;
    let t10835 = t7390 * t10834;
    let t10836 = 0.14896037479937677779e-1_f64 * t10835;
    let t10837 = t7354 * t3488;
    let t10838 = t2684 * t10837;
    let t10839 = 0.25561950635947166451e0_f64 * t10838;
    let t10840 = t2365 * t8769;
    let t10841 = t6111 * t10840;
    let t10842 = 0.29792074959875355558e-1_f64 * t10841;
    let t10843 = t826 * t3488;
    (t10829, t10830, t10835, t10836, t10838, t10839, t10841, t10842, t10843)
}
