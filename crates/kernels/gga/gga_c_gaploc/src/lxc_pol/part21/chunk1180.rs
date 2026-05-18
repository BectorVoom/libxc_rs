//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1180/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1180<F: Float>(t31889: F, t2268: F, t6320: F, t6509: F, t8097: F, t20117: F, t2854: F, t10246: F, t6313: F, t123: F, t25760: F, t2326: F, t9074: F) -> (F, F, F, F, F) {
    let t31890 = F::new(0.11856252764865062333e-2) * t31889;
    let t31894 = F::new(0.34146007962811379518e0) * t2268 * t6320 * t8097 * t6509;
    let t31898 = F::new(0.34146007962811379518e0) * t2268 * t6320 * t2854 * t20117;
    let t31900 = F::new(0.53116012386595479252e0) * t6313 * t10246;
    let t31903 = t25760 * t123;
    let t31905 = t9074 * t31903 * t2326;
    (t31890, t31894, t31898, t31900, t31905)
}
