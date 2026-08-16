//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1172/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1172<F: Float>(t23726: F, t3327: F, t2293: F, t7995: F, t2268: F, t2343: F, t10145: F, t6313: F, t6776: F, t988: F, t25893: F, t6520: F) -> (F, F, F, F, F, F) {
    let t31710 = F::cast_from(0.7588001769513639893e-1_f64) * t23726 * t3327;
    let t31711 = t7995 * t2293;
    let t31714 = F::cast_from(0.1138200265427045984e0_f64) * t2268 * t2343 * t31711;
    let t31724 = F::cast_from(0.15176003539027279786e0_f64) * t6313 * t10145;
    let t31727 = F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t6776 * t988;
    let t31735 = t25893 * t6520;
    (t31710, t31711, t31714, t31724, t31727, t31735)
}
