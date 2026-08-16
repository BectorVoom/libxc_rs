//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1671/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1671(t6205: f64, t15421: f64, t23565: f64, t11299: f64, t88031: f64, t935: f64, t23550: f64, t52224: f64, t11452: f64, t11466: f64, t1622: f64, t19173: f64, t23714: f64, t23717: f64, t23776: f64, t2987: f64, t41238: f64, t41658: f64, t41667: f64, t4685: f64, t52642: f64, t52825: f64, t6158: f64, t6174: f64, t6177: f64, t64060: f64, t64319: f64, t78108: f64, t88008: f64, t88055: f64, t88140: f64, t88264: f64, t88291: f64, t88305: f64, t88321: f64, t88336: f64, t946: f64, t954: f64, t965: f64, t973: f64) -> (f64, f64, f64, f64, f64) {
    let t88351 = t6205 * t6205;
    let t88358 = 24.0_f64 * t15421 * t23565;
    let t88361 = 24.0_f64 * t11299 * t88031 * t935;
    let t88363 = 0.2069040516770936012e4_f64 * t52224 * t23550;
    let t88364 = t88140 + 0.23392894490538584828e1_f64 * t4685 * t23714 + 0.5848223622634646207e0_f64 * t965 * t88264 * t973 + 0.4101607543286562663e4_f64 * t52642 * t23717 + 0.91082604192152556044e5_f64 * t41658 * t88008 * t41238 + 0.82761620670837440481e4_f64 * t52825 * t23776 - 0.24828486201251232145e5_f64 * t41667 * t88055 * t11452 + 1.0_f64 * t946 * (t88291 + t88305 + t88321 + t88336) * t954 + 4.0_f64 * t78108 * t1622 + 6.0_f64 * t19173 * t6174 + 0.1929837539843104208e3_f64 * t64060 * t6177 - 0.14035736694323150897e2_f64 * t11466 * t88008 * t973 - 0.35089341735807877242e1_f64 * t2987 * t88351 * t973 - 12.0_f64 * t64319 * t6158 - t88358 + t88361 - t88363;
    (t88351, t88358, t88361, t88363, t88364)
}
