//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 724/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk724(t12223: f64, t723: f64, t1445: f64, t12214: f64, t12213: f64, t701: f64, t10927: f64, t10934: f64, t10935: f64, t10937: f64, t12207: f64, t12210: f64, t12215: f64, t12220: f64, t1998: f64, t2009: f64, t2087: f64, t2103: f64, t780: f64, t813: f64, t833: f64, t9935: f64, t9937: f64, t9942: f64, t9946: f64) -> f64 {
    let t12224 = t12223 * t723;
    let t12225 = t1445 * t12224;
    let t12228 = t1445 * t12214;
    let t12231 = t12213 * t701;
    let t12232 = t1445 * t12231;
    let t12235 = -t10927 + t10934 + 0.35750489951850426669e0_f64 * t780 * t12207 - 0.35750489951850426669e0_f64 * t12210 * t2009 + 0.71500979903700853338e0_f64 * t2103 * t12215 - 0.69017266717057349418e1_f64 * t2087 * t12220 - 0.46011511144704899612e1_f64 * t813 * t12225 + 0.11502877786176224903e2_f64 * t833 * t12228 - 0.23005755572352449806e1_f64 * t1998 * t12232 + t9935 + t9937 - t9942 - t9946 + t10935 + t10937;
    t12235
}
