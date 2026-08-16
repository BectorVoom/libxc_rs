//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1475/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1475(t1174: f64, t15569: f64, t18321: f64, t22119: f64, t22154: f64, t3555: f64, t3577: f64, t3578: f64, t44805: f64, t44817: f64, t44938: f64, t4889: f64, t53490: f64, t5975: f64, t5979: f64, t6178: f64, t6192: f64, t6219: f64, t65884: f64, t66622: f64, t66668: f64, t73142: f64, t75836: f64, t75847: f64, t974: f64) -> f64 {
    let t79387 = -10.0_f64 / 243.0_f64 * t53490 - 19.0_f64 / 216.0_f64 * t66622 * t6192 - t3577 * t3578 * t6219 * t5979 / 768.0_f64 - t3577 * t3578 * t6219 * t5975 / 384.0_f64 - 154.0_f64 / 243.0_f64 * t73142 + 22.0_f64 / 81.0_f64 * t18321 * t6178 - t1174 * t974 * t3555 * t75847 / 48.0_f64 + t1174 * t974 * t44938 * t75836 / 6.0_f64 - 7.0_f64 / 54.0_f64 * t1174 * t974 * t44817 * t75836 + 35.0_f64 / 972.0_f64 * t1174 * t974 * t44805 * t75836 + t65884 * t6192 / 36.0_f64 + t15569 * t22154 / 72.0_f64 + 2.0_f64 / 9.0_f64 * t4889 * t22119 + t66668 / 216.0_f64;
    t79387
}
