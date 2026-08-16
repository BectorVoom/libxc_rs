//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1188/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1188(t25841: f64, t874: f64, t2268: f64, t2343: f64, t10153: f64, t10223: f64, t1624: f64, t31825: f64, t31829: f64, t31835: f64, t31838: f64, t31840: f64, t31842: f64, t31846: f64, t31849: f64, t31853: f64, t31856: f64, t3340: f64, t535: f64, t6305: f64) -> (f64, f64) {
    let t31857 = t25841 * t874;
    let t31860 = 0.56910013271352299198e-1_f64 * t2268 * t2343 * t31857;
    let t31861 = 0.28455006635676149599e-1_f64 * t2268 * t1624 * t3340 + 0.56910013271352299198e-1_f64 * t2268 * t535 * t10223 - t31825 + 0.1138200265427045984e0_f64 * t6305 * t10153 + 0.1138200265427045984e0_f64 * t2268 * t2343 * t31829 + t31835 + t31838 - t31840 - t31842 + t31846 - t31849 - t31853 - t31856 + t31860;
    (t31857, t31861)
}
