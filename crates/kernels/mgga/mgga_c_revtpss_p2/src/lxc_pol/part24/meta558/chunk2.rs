//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1671/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1671<F: Float>(t6205: F, t15421: F, t23565: F, t11299: F, t88031: F, t935: F, t23550: F, t52224: F, t11452: F, t11466: F, t1622: F, t19173: F, t23714: F, t23717: F, t23776: F, t2987: F, t41238: F, t41658: F, t41667: F, t4685: F, t52642: F, t52825: F, t6158: F, t6174: F, t6177: F, t64060: F, t64319: F, t78108: F, t88008: F, t88055: F, t88140: F, t88264: F, t88291: F, t88305: F, t88321: F, t88336: F, t946: F, t954: F, t965: F, t973: F) -> (F, F, F, F, F) {
    let t88351 = t6205 * t6205;
    let t88358 = F::new(24.0) * t15421 * t23565;
    let t88361 = F::new(24.0) * t11299 * t88031 * t935;
    let t88363 = F::cast_from(0.2069040516770936012e4_f64) * t52224 * t23550;
    let t88364 = t88140 + F::cast_from(0.23392894490538584828e1_f64) * t4685 * t23714 + F::cast_from(0.5848223622634646207e0_f64) * t965 * t88264 * t973 + F::cast_from(0.4101607543286562663e4_f64) * t52642 * t23717 + F::cast_from(0.91082604192152556044e5_f64) * t41658 * t88008 * t41238 + F::cast_from(0.82761620670837440481e4_f64) * t52825 * t23776 - F::cast_from(0.24828486201251232145e5_f64) * t41667 * t88055 * t11452 + F::new(1.0) * t946 * (t88291 + t88305 + t88321 + t88336) * t954 + F::new(4.0) * t78108 * t1622 + F::new(6.0) * t19173 * t6174 + F::cast_from(0.1929837539843104208e3_f64) * t64060 * t6177 - F::cast_from(0.14035736694323150897e2_f64) * t11466 * t88008 * t973 - F::cast_from(0.35089341735807877242e1_f64) * t2987 * t88351 * t973 - F::new(12.0) * t64319 * t6158 - t88358 + t88361 - t88363;
    (t88351, t88358, t88361, t88363, t88364)
}
