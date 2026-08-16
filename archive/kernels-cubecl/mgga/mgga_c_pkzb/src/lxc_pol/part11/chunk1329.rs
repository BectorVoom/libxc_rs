//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1329/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1329<F: Float>(t10200: F, t3214: F, t10251: F, t10258: F, t10267: F, t1167: F, t23345: F, t23355: F, t23367: F, t27085: F, t28138: F, t28283: F, t28295: F, t28303: F, t28305: F, t28316: F, t3026: F, t3061: F, t3235: F, t758: F) -> F {
    let t32204 = t3214 * t10200;
    let t32208 = -F::cast_from(0.41159057393346947494e-1_f64) * t10258 * t10267 + F::cast_from(0.38586616306262763276e-2_f64) * t3235 * t758 * t27085 * t1167 + F::cast_from(0.38586616306262763276e-2_f64) * t3235 * t758 * t10251 * t3026 - F::cast_from(0.1543464652250510531e-1_f64) * t3235 * t758 * t28138 * t3061 + F::cast_from(0.42874018118069736972e-3_f64) * t28283 - F::cast_from(0.45732285992607719436e-2_f64) * t28295 + F::cast_from(0.19055119163586549765e-3_f64) * t23345 - F::cast_from(0.14291339372689912324e-3_f64) * t28303 + F::cast_from(0.14481890564325777821e-1_f64) * t28305 - F::cast_from(0.22866142996303859718e-2_f64) * t32204 - F::cast_from(0.10162730220579493208e-2_f64) * t23355 - F::cast_from(0.85748036236139473947e-3_f64) * t28316 + t23367;
    t32208
}
