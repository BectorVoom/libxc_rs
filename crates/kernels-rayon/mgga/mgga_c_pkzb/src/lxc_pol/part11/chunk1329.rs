//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1329/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1329(t10200: f64, t3214: f64, t10251: f64, t10258: f64, t10267: f64, t1167: f64, t23345: f64, t23355: f64, t23367: f64, t27085: f64, t28138: f64, t28283: f64, t28295: f64, t28303: f64, t28305: f64, t28316: f64, t3026: f64, t3061: f64, t3235: f64, t758: f64) -> f64 {
    let t32204 = t3214 * t10200;
    let t32208 = -0.41159057393346947494e-1_f64 * t10258 * t10267 + 0.38586616306262763276e-2_f64 * t3235 * t758 * t27085 * t1167 + 0.38586616306262763276e-2_f64 * t3235 * t758 * t10251 * t3026 - 0.1543464652250510531e-1_f64 * t3235 * t758 * t28138 * t3061 + 0.42874018118069736972e-3_f64 * t28283 - 0.45732285992607719436e-2_f64 * t28295 + 0.19055119163586549765e-3_f64 * t23345 - 0.14291339372689912324e-3_f64 * t28303 + 0.14481890564325777821e-1_f64 * t28305 - 0.22866142996303859718e-2_f64 * t32204 - 0.10162730220579493208e-2_f64 * t23355 - 0.85748036236139473947e-3_f64 * t28316 + t23367;
    t32208
}
