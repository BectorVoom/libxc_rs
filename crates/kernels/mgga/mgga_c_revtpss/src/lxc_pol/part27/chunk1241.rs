//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1241/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1241<F: Float>(t1940: F, t1963: F, t2000: F, t2403: F, t25436: F, t25440: F, t25445: F, t25752: F, t25767: F, t25778: F, t25781: F, t27158: F, t4541: F, t7087: F, t7091: F, t7200: F, t7207: F, t92742: F, t92775: F, t92822: F, t93404: F, t94276: F, t94280: F, t94286: F, t94293: F, t94297: F, t94312: F, t94316: F, t94320: F) -> F {
    let t94324 = F::new(9.0) * t4541 * t7087 * t25752 - F::new(3.0) / F::new(2.0) * t1940 * t7091 * t94276 + F::new(9.0) * t27158 * t94280 - F::new(3.0) / F::new(2.0) * t1940 * t92775 * t7207 - F::new(3.0) / F::new(2.0) * t1940 * t7091 * t94286 + F::new(3.0) * t1940 * t93404 * t25778 + F::new(9.0) / F::new(2.0) * t2403 * t1963 * t94293 + F::new(9.0) / F::new(2.0) * t2403 * t1963 * t94297 + F::new(9.0) / F::new(2.0) * t2403 * t25436 * t7200 + F::new(9.0) / F::new(2.0) * t2403 * t7087 * t25767 - F::new(3.0) * t1940 * t25440 * t25781 + F::new(3.0) * t92822 * t2000 - F::new(3.0) * t1940 * t92742 * t94312 + F::new(3.0) * t1940 * t25445 * t94316 + F::new(3.0) / F::new(2.0) * t2403 * t1963 * t94320;
    t94324
}
