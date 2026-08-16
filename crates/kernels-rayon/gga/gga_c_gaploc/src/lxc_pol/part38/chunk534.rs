//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 534/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk534(t10122: f64, t550: f64, t1365: f64, t3355: f64, t6313: f64, t3347: f64, t3344: f64, t484: f64, t874: f64, t986: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10231 = t550 * t10122;
    let t10232 = t1365 * t10231;
    let t10236 = 0.7588001769513639893e-1_f64 * t6313 * t3355;
    let t10238 = 0.1138200265427045984e0_f64 * t6313 * t3347;
    let t10239 = t484 * t3344;
    let t10240 = 0.15808337019820083111e-2_f64 * t10239;
    let t10241 = t874 * t986;
    (t10231, t10232, t10236, t10238, t10239, t10240, t10241)
}
