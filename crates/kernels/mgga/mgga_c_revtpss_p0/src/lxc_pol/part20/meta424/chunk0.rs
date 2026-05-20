//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1591/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1591<F: Float>(t406: F, t43822: F, t12254: F, t141: F, t43835: F, t1145: F, t43843: F, t1139: F, t43908: F, t3407: F, t43825: F, t43886: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t43899: F, t43902: F, t43905: F) -> (F, F, F, F, F, F) {
    let t43946 = F::powf(t406, -F::new(0.25e1));
    let t43947 = t43946 * t43822;
    let t43950 = t141 * t12254 * t43835;
    let t43953 = t141 * t1145 * t43843;
    let t43955 = t1139 * t43908;
    let t43957 = t3407 * t43825;
    let t43959 = F::cast_from(0.40256666666666666666e1_f64) * t43886 - F::cast_from(0.12524296296296296297e1_f64) * t43888 + F::cast_from(0.80513333333333333336e0_f64) * t43890 + F::cast_from(0.16102666666666666667e1_f64) * t43892 - F::new(0.24154e1) * t43894 - F::cast_from(0.40256666666666666668e0_f64) * t43896 - F::new(0.72462e1) * t43899 + F::new(0.72462e1) * t43902 + F::new(0.301925e0) * t43905 + F::cast_from(0.6189328125e-1_f64) * t43947 + F::new(0.22076e0) * t43950 + F::new(0.298026e1) * t43953 + F::new(0.16504875e0) * t43955 + F::cast_from(0.247573125e0_f64) * t43957;
    (t43947, t43950, t43953, t43955, t43957, t43959)
}
