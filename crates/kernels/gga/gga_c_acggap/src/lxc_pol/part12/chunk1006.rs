//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1006/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1006<F: Float>(t2404: F, t7924: F, t33839: F, t33841: F, t33843: F, t33851: F, t33857: F, t33859: F, t33861: F, t33867: F, t33869: F, t33823: F, t33827: F, t33831: F, t33835: F, t33847: F, t33853: F, t33863: F, t33865: F) -> (F, F) {
    let t36811 = t7924 * t2404;
    let t36817 = 0.31448092289604152068e-2 * t33839;
    let t36818 = 0.37737710747524982482e-2 * t33841;
    let t36819 = 0.62896184579208304138e-3 * t33843;
    let t36821 = 0.41930789719472202758e-3 * t33851;
    let t36823 = 0.12579236915841660827e-2 * t33857;
    let t36824 = 11.0 / 288.0 * t33859;
    let t36825 = 35.0 / 216.0 * t33861;
    let t36828 = 0.85748036236139473944e-3 * t33867;
    let t36829 = 0.15724046144802076034e-2 * t33869;
    let t36830 = 0.62896184579208304136e-2 * t33823 - 0.94344276868812456204e-2 * t33827 - 0.12579236915841660828e-2 * t33831 - 0.18868855373762491241e-1 * t33835 - t36817 - t36818 + t36819 + 0.31448092289604152069e-3 * t33847 + t36821 + 0.41930789719472202758e-3 * t33853 + t36823 + t36824 - t36825 - t33863 / 24.0 + 0.51448821741683684366e-2 * t33865 - t36828 + t36829;
    (t36811, t36830)
}
