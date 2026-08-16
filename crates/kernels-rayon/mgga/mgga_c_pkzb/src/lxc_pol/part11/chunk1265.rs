//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1265/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1265(t30213: f64, t30216: f64, t30219: f64, t30221: f64, t30223: f64, t30225: f64, t30227: f64, t30230: f64, t30234: f64, t30236: f64, t30238: f64, t30242: f64, t30245: f64, t30248: f64, t30252: f64, t30255: f64, t30259: f64, t30261: f64, t30263: f64, t30265: f64, t30268: f64, t30270: f64) -> (f64, f64) {
    let t30991 = t30213 + t30216 + t30219 - t30221 + t30223 - t30225 + t30227 - t30230 - t30234 + t30236 + t30238;
    let t30993 = -t30242 - t30245 - t30248 + t30252 + t30255 + t30259 + t30261 - t30263 - t30265 - t30268 - t30270;
    (t30991, t30993)
}
