//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1186/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1186<F: Float>(t1063: F, t2343: F, t6519: F, t7995: F, t10243: F, t6313: F, t6750: F, t7930: F, t1222: F, t3351: F, t2321: F, t8289: F, t882: F) -> (F, F, F, F, F) {
    let t31956 = F::cast_from(0.1138200265427045984e0_f64) * t1063 * t2343 * t7995 * t6519;
    let t31958 = F::cast_from(0.7588001769513639893e-1_f64) * t6313 * t10243;
    let t31961 = F::cast_from(0.17073003981405689759e0_f64) * t1063 * t7930 * t6750;
    let t31965 = t1222 * t3351;
    let t31966 = F::cast_from(0.31616674039640166222e-2_f64) * t31965;
    let t31968 = t882 * t8289 * t2321;
    (t31956, t31958, t31961, t31966, t31968)
}
