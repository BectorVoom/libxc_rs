//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 829/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk829<F: Float>(t16907: F, t554: F, t2057: F, t4702: F, t12374: F, t16827: F, t16830: F, t16889: F, t16891: F, t16894: F, t16897: F, t16902: F, t2001: F, t3380: F, t399: F, t4674: F, t4677: F, t4700: F, t4704: F, t538: F, t555: F) -> F {
    let t16908 = t16907 * t554;
    let t16911 = t2057 * t4702;
    let t16917 = -t16827 - F::new(2.0) * t4674 * t555 + F::new(2.0) * t16830 + F::new(2.0) * t16889 + F::new(4.0) * t16891 * t3380 - F::new(4.0) * t2001 * t16894 - F::new(0.1208182677680765956e1) * t16897 * t399 + F::new(0.1208182677680765956e1) * t4700 * t399 - F::new(0.1208182677680765956e1) * t16902 * t399 + F::new(0.1208182677680765956e1) * t4704 * t399 - F::new(2.0) * t2001 * t16908 + F::new(4.0) * t2001 * t16911 * t538 - F::new(4.0) * t12374 * t4677;
    t16917
}
