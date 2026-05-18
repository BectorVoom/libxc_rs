//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 845/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk845<F: Float>(t2268: F, t2765: F, t34267: F, t13310: F, t2312: F, t42825: F, t1063: F, t11259: F, t6320: F, t6519: F, t2854: F, t31585: F) -> (F, F, F, F, F) {
    let t44527 = F::new(0.39837009289946609438e0) * t2268 * t2765 * t34267;
    let t44528 = t2312 * t13310;
    let t44529 = F::new(0.11856252764865062333e-2) * t44528;
    let t44530 = F::new(0.12646669615856066489e-1) * t42825;
    let t44534 = F::new(0.17073003981405689759e0) * t1063 * t6320 * t11259 * t6519;
    let t44538 = F::new(0.34146007962811379518e0) * t2268 * t6320 * t2854 * t31585;
    (t44527, t44529, t44530, t44534, t44538)
}
