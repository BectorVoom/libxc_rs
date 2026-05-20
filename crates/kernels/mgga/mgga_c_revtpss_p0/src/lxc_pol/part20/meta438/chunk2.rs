//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1653/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1653<F: Float>(t43886: F, t43888: F, t43890: F, t43892: F, t43894: F, t43896: F, t43899: F, t43902: F, t43905: F, t43947: F, t43950: F, t43953: F, t43955: F, t43957: F) -> F {
    let t45149 = F::cast_from(0.68863333333333333334e1_f64) * t43886 - F::cast_from(0.21424148148148148148e1_f64) * t43888 + F::cast_from(0.13772666666666666666e1_f64) * t43890 + F::cast_from(0.27545333333333333333e1_f64) * t43892 - F::new(0.41318e1) * t43894 - F::cast_from(0.68863333333333333332e0_f64) * t43896 - F::new(0.123954e2) * t43899 + F::new(0.123954e2) * t43902 + F::new(0.516475e0) * t43905 + F::cast_from(0.2366859375e0_f64) * t43947 + F::cast_from(0.27785333333333333334e0_f64) * t43950 + F::new(0.375102e1) * t43953 + F::new(0.6311625e0) * t43955 + F::new(0.94674375e0) * t43957;
    t45149
}
