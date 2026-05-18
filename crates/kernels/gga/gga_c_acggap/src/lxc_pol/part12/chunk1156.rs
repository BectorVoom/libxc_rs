//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1156/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1156<F: Float>(t33869: F, t33823: F, t33827: F, t33831: F, t33835: F, t33847: F, t33853: F, t33863: F, t33865: F, t36817: F, t36818: F, t36819: F, t36821: F, t36823: F, t36824: F, t36825: F, t36828: F) -> F {
    let t36829 = F::new(0.15724046144802076034e-2) * t33869;
    let t36830 = F::new(0.62896184579208304136e-2) * t33823 - F::new(0.94344276868812456204e-2) * t33827 - F::new(0.12579236915841660828e-2) * t33831 - F::new(0.18868855373762491241e-1) * t33835 - t36817 - t36818 + t36819 + F::new(0.31448092289604152069e-3) * t33847 + t36821 + F::new(0.41930789719472202758e-3) * t33853 + t36823 + t36824 - t36825 - t33863 / F::new(24.0) + F::new(0.51448821741683684366e-2) * t33865 - t36828 + t36829;
    t36830
}
