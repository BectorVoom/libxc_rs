//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1599/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1599<F: Float>(t43886: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t43899: F, t43902: F, t43905: F, t43947: F, t43950: F, t43953: F, t43955: F, t43957: F) -> F {
    let t44082 = F::cast_from(0.39862222222222222223e1_f64) * t43886 - F::cast_from(0.12401580246913580247e1_f64) * t43888 + F::cast_from(0.79724444444444444446e0_f64) * t43890 + F::cast_from(0.15944888888888888889e1_f64) * t43892 - F::cast_from(0.23917333333333333333e1_f64) * t43894 - F::cast_from(0.39862222222222222223e0_f64) * t43896 - F::cast_from(0.71752000000000000002e1_f64) * t43899 + F::new(0.71752e1) * t43902 + F::cast_from(0.29896666666666666667e0_f64) * t43905 + F::cast_from(0.1151859375e0_f64) * t43947 + F::cast_from(0.21908444444444444444e0_f64) * t43950 + F::new(0.295764e1) * t43953 + F::new(0.3071625e0) * t43955 + F::new(0.46074375e0) * t43957;
    t44082
}
