//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 692/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk692(t6471: f64, t882: f64, t493: f64, t6393: f64, t492: f64, t105: f64, t1063: f64, t1083: f64, t2268: f64, t2305: f64, t2350: f64, t380: f64, t3822: f64, t6305: f64, t6313: f64, t6425: f64, t6430: f64, t6433: f64, t6438: f64, t6444: f64, t6448: f64, t6451: f64, t6457: f64, t6460: f64, t6463: f64, t6468: f64, t889: f64) -> (f64, f64, f64) {
    let t6472 = t882 * t6471;
    let t6474 = t493 * t6393;
    let t6475 = t492 * t6474;
    let t6482 = 0.28455006635676149599e-1_f64 * t1063 * t6425 + 0.1138200265427045984e0_f64 * t2268 * t6430 + 0.28455006635676149599e-1_f64 * t2268 * t6433 - 0.1707300398140568976e0_f64 * t6305 * t2305 + 0.56910013271352299198e-1_f64 * t3822 * t6438 - 0.22764005308540919679e0_f64 * t6313 * t2305 + 0.56910013271352299198e-1_f64 * t2268 * t6444 - 0.1707300398140568976e0_f64 * t2268 * t6448 - 0.23712505529730124666e-2_f64 * t6451 + 0.23712505529730124666e-2_f64 * t6457 + 0.11856252764865062333e-2_f64 * t6460 - 0.56910013271352299198e-1_f64 * t1063 * t6463 - 0.11856252764865062333e-2_f64 * t6468 - 0.35568758294595186999e-2_f64 * t6472 - 0.28455006635676149599e-1_f64 * t105 * t6475 - 0.7588001769513639893e-1_f64 * t1083 * t889 - 0.7588001769513639893e-1_f64 * t380 * t2350;
    (t6472, t6474, t6482)
}
