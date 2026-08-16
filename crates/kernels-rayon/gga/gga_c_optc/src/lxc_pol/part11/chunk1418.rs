//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1418/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1418(t5154: f64, t1067: f64, t1075: f64, t1094: f64, t12223: f64, t12238: f64, t17803: f64, t17835: f64, t17843: f64, t17848: f64, t26593: f64, t26665: f64, t26667: f64, t2935: f64, t2974: f64, t2976: f64, t3035: f64, t3059: f64, t3061: f64, t34801: f64, t4087: f64, t44914: f64, t45045: f64, t5123: f64, t5158: f64, t58311: f64, t58880: f64, t59263: f64, t59281: f64, t59294: f64, t59310: f64, t59325: f64, t8765: f64, t8772: f64, t8786: f64, t8848: f64, t8850: f64) -> f64 {
    let t59348 = t5154 * t5154;
    let t59367 = 0.19298809906722418784e3_f64 * t44914 * t5158 + 4.0_f64 * t4087 * t17835 - 0.24829604254387158296e5_f64 * t26593 * t59263 * t8850 + 1.0_f64 * t1067 * (t59281 + t59294 + t59310 + t59325) * t1075 + 0.19965908856856833625e6_f64 * t26665 * t59263 * t26667 - 0.1403573615389248977e2_f64 * t8765 * t58311 * t1094 - 0.35089340384731224426e1_f64 * t3035 * t58880 * t1094 + 0.51947267698127589897e2_f64 * t3059 * t58880 * t3061 + 24.0_f64 * t12238 * t17848 - 24.0_f64 * t8786 * t59263 * t1075 - 6.0_f64 * t2935 * t59348 * t1075 + 0.96494049533612093922e2_f64 * t2974 * t59348 * t2976 + 0.6233672123775310788e3_f64 * t8772 * t58311 * t3061 - 12.0_f64 * t45045 * t5123 - 0.77195239626889675138e3_f64 * t34801 * t17803 + 0.11579285944033451271e4_f64 * t8848 * t59263 * t2976 + 0.14035736153892489771e2_f64 * t12223 * t17843;
    t59367
}
