//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1052/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1052<F: Float>(t10153: F, t6940: F, t1403: F, t27938: F, t681: F, t2568: F, t27889: F, t766: F, t10002: F, t27983: F, t51892: F, t6175: F, t24237: F, t28002: F, t1168: F, t97304: F) -> (F, F, F, F, F, F, F) {
    let t107954 = t10153 * t6940;
    let t107958 = t1403 * t681 * t27938 / 9.0;
    let t107964 = t2568 * t27889 * t766;
    let t107966 = t10002 * t27983;
    let t107968 = t51892 * t6175;
    let t107971 = t24237 * t28002 / 27.0;
    let t107976 = t97304 * t1168;
    (t107954, t107958, t107964, t107966, t107968, t107971, t107976)
}
