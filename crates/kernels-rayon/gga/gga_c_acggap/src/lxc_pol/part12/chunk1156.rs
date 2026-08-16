//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1156/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1156(t33869: f64, t33823: f64, t33827: f64, t33831: f64, t33835: f64, t33847: f64, t33853: f64, t33863: f64, t33865: f64, t36817: f64, t36818: f64, t36819: f64, t36821: f64, t36823: f64, t36824: f64, t36825: f64, t36828: f64) -> f64 {
    let t36829 = 0.15724046144802076034e-2_f64 * t33869;
    let t36830 = 0.62896184579208304136e-2_f64 * t33823 - 0.94344276868812456204e-2_f64 * t33827 - 0.12579236915841660828e-2_f64 * t33831 - 0.18868855373762491241e-1_f64 * t33835 - t36817 - t36818 + t36819 + 0.31448092289604152069e-3_f64 * t33847 + t36821 + 0.41930789719472202758e-3_f64 * t33853 + t36823 + t36824 - t36825 - t33863 / 24.0_f64 + 0.51448821741683684366e-2_f64 * t33865 - t36828 + t36829;
    t36830
}
