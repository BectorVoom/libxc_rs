//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1301/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1301(t10370: f64, t4614: f64, t574: f64, t2859: f64, t30949: f64, t10447: f64, t1562: f64, t10324: f64, t1641: f64, t10365: f64, t4953: f64, t1445: f64, t26428: f64, t874: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34181 = 0.12269736305254639897e2_f64 * t574 * t4614 * t10370;
    let t34186 = 0.14300195980740170668e1_f64 * t2859 * t30949;
    let t34189 = 0.18404604457881959845e2_f64 * t1562 * t4614 * t10447;
    let t34191 = 0.12269736305254639897e2_f64 * t1641 * t10324;
    let t34216 = 0.13803453343411469884e2_f64 * t4953 * t10365;
    let t34220 = 0.69017266717057349418e1_f64 * t1562 * t1445 * t26428 * t874;
    (t34181, t34186, t34189, t34191, t34216, t34220)
}
