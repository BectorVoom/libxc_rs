//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 987/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk987<F: Float>(t34893: F, t7440: F, t8929: F, t2282: F, t7600: F, t174: F, t7815: F, t1181: F, t20992: F, t7351: F, t7426: F, t1983: F, t30127: F, t7586: F, t8791: F) -> (F, F, F, F, F, F) {
    let t34894 = F::new(0.16809375e0) * t34893;
    let t34895 = t7440 * t8929;
    let t34896 = F::new(0.16809375e0) * t34895;
    let t34897 = t7600 * t2282;
    let t34903 = t7815 * t174;
    let t34945 = t7426 * t1181 * t7351 * t20992;
    let t34946 = F::cast_from(0.18868855373762491241e-2_f64) * t34945;
    let t34957 = t30127 * t7586 * t1983 * t8791;
    (t34894, t34896, t34897, t34903, t34946, t34957)
}
