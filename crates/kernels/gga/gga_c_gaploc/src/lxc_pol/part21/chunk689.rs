//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 689/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk689<F: Float>(t6471: F, t882: F, t493: F, t6393: F, t492: F, t105: F, t1063: F, t1083: F, t2268: F, t2305: F, t2350: F, t380: F, t3822: F, t6305: F, t6313: F, t6425: F, t6430: F, t6433: F, t6438: F, t6444: F, t6448: F, t6451: F, t6457: F, t6460: F, t6463: F, t6468: F, t889: F) -> (F, F, F) {
    let t6472 = t882 * t6471;
    let t6474 = t493 * t6393;
    let t6475 = t492 * t6474;
    let t6482 = F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t6425 + F::cast_from(0.1138200265427045984e0_f64) * t2268 * t6430 + F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t6433 - F::cast_from(0.1707300398140568976e0_f64) * t6305 * t2305 + F::cast_from(0.56910013271352299198e-1_f64) * t3822 * t6438 - F::cast_from(0.22764005308540919679e0_f64) * t6313 * t2305 + F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t6444 - F::cast_from(0.1707300398140568976e0_f64) * t2268 * t6448 - F::cast_from(0.23712505529730124666e-2_f64) * t6451 + F::cast_from(0.23712505529730124666e-2_f64) * t6457 + F::cast_from(0.11856252764865062333e-2_f64) * t6460 - F::cast_from(0.56910013271352299198e-1_f64) * t1063 * t6463 - F::cast_from(0.11856252764865062333e-2_f64) * t6468 - F::cast_from(0.35568758294595186999e-2_f64) * t6472 - F::cast_from(0.28455006635676149599e-1_f64) * t105 * t6475 - F::cast_from(0.7588001769513639893e-1_f64) * t1083 * t889 - F::cast_from(0.7588001769513639893e-1_f64) * t380 * t2350;
    (t6472, t6474, t6482)
}
