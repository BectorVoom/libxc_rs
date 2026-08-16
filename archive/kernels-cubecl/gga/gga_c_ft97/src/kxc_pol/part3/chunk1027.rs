//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 1027/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk1027<F: Float>(t19372: F, t19428: F, t19478: F, t19531: F, t19584: F, t19791: F, t19809: F, t19880: F, t1218: F, t1253: F, t18987: F, t18989: F, t18992: F, t19308: F, t301: F, t317: F, t4027: F, t4135: F, t4309: F, t5207: F, t5305: F, t5422: F, t830: F, t880: F) -> F {
    let t19883 = t19372 + t19428 + t19478 + t19531 + t19584 + t19791 + t19809 + t19880;
    let t19885 = -F::cast_from(2.0_f64) * t1218 * t4309 - F::cast_from(2.0_f64) * t1253 * t4027 - F::cast_from(2.0_f64) * t1253 * t4135 - t18987 * t317 - t18989 * t317 - t18992 * t317 - t19308 * t317 - t19883 * t301 - t5207 * t880 - t5305 * t880 - t5422 * t830;
    t19885
}
