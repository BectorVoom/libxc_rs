//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1052/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1052<F: Float>(t2293: F, t7995: F, t2268: F, t2343: F, t10145: F, t6313: F, t6776: F, t988: F, t25893: F, t6520: F, t23763: F, t25722: F, t6508: F, t4261: F, t9074: F, t19532: F, t25723: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31711 = t7995 * t2293;
    let t31714 = 0.1138200265427045984e0 * t2268 * t2343 * t31711;
    let t31724 = 0.15176003539027279786e0 * t6313 * t10145;
    let t31727 = 0.28455006635676149599e-1 * t2268 * t6776 * t988;
    let t31735 = t25893 * t6520;
    let t31737 = 0.18970004423784099733e-1 * t23763 * t31735;
    let t31752 = t6508 * t25722;
    let t31754 = t9074 * t4261 * t31752;
    let t31755 = 0.142275033178380748e-1 * t31754;
    let t31757 = t9074 * t19532 * t25723;
    (t31711, t31714, t31724, t31727, t31735, t31737, t31752, t31755, t31757)
}
