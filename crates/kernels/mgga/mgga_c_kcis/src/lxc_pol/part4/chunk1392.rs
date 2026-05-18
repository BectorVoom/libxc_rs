//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1392/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1392<F: Float>(t1889: F, t4457: F, t12605: F, t12827: F, t12830: F, t12834: F, t12840: F, t12842: F, t12846: F, t12849: F, t18069: F, t18071: F, t18080: F, t18083: F, t4439: F) -> F {
    let t18086 = t1889 * t4457;
    let t18087 = t12605 * t18086;
    let t18090 = -t12846 / F::new(864.0) + t18069 / F::new(324.0) + t4439 * t18071 / F::new(96.0) + t12840 - t12827 / F::new(1296.0) + t12830 / F::new(1728.0) + t12834 / F::new(1296.0) - t12849 / F::new(864.0) + t12842 / F::new(432.0) + t4439 * t18080 / F::new(72.0) - t4439 * t18083 / F::new(72.0) + t4439 * t18087 / F::new(288.0);
    t18090
}
