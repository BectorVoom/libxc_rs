//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1189/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1189<F: Float>(t10243: F, t6305: F, t26011: F, t3822: F, t894: F, t1063: F, t20395: F, t2343: F, t2787: F, t3351: F, t6338: F, t10160: F, t23927: F) -> (F, F, F, F, F) {
    let t32059 = F::new(0.56910013271352299198e-1) * t6305 * t10243;
    let t32062 = F::new(0.56910013271352299198e-1) * t3822 * t894 * t26011;
    let t32066 = F::new(0.1138200265427045984e0) * t1063 * t2343 * t2787 * t20395;
    let t32071 = t6338 * t3351;
    let t32072 = F::new(0.11856252764865062333e-2) * t32071;
    let t32073 = t23927 * t10160;
    (t32059, t32062, t32066, t32072, t32073)
}
