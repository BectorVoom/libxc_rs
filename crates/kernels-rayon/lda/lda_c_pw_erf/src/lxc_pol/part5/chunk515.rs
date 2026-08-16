//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 515/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk515(t2460: f64, t2535: f64, t2572: f64, t2579: f64, t1197: f64, t1202: f64, t1209: f64, t1213: f64, t153: f64, t1540: f64, t156: f64, t168: f64, t2240: f64, t2244: f64, t2249: f64, t2298: f64, t2357: f64, t2379: f64, t242: f64, t245: f64) -> (f64, f64) {
    let t2581 = t2460 + t2535 + t2572 + t2579;
    let t2589 = -t1197 + 0.1675256410710088_f64 * t2240 + t1202 - 0.0837628205355044_f64 * t2379 * t242 - 0.1675256410710088_f64 * t2244 - t1209 - t1213 + 0.039794582218349216_f64 * t2249 - 0.011938374665504766_f64 * t168 * t245 * t2581 + t1540 - 1.1389037339096726_f64 * t2298 + 0.42708890021612717_f64 * t153 * t156 * t2357;
    (t2581, t2589)
}
