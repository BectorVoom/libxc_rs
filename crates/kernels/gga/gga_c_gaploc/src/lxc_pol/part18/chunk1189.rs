//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1189/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1189<F: Float>(t31889: F, t2268: F, t6320: F, t6509: F, t8097: F, t20117: F, t2854: F, t10246: F, t6313: F, t10124: F, t10153: F, t1064: F, t31863: F, t31865: F, t31869: F, t31870: F, t31879: F, t31881: F, t31883: F, t31886: F, t3818: F, t3822: F) -> F {
    let t31890 = F::cast_from(0.11856252764865062333e-2_f64) * t31889;
    let t31894 = F::cast_from(0.34146007962811379518e0_f64) * t2268 * t6320 * t8097 * t6509;
    let t31898 = F::cast_from(0.34146007962811379518e0_f64) * t2268 * t6320 * t2854 * t20117;
    let t31900 = F::cast_from(0.53116012386595479252e0_f64) * t6313 * t10246;
    let t31901 = t31863 + t31865 + t31869 - F::cast_from(0.56910013271352299198e-1_f64) * t3822 * t1064 * t31870 + F::cast_from(0.7588001769513639893e-1_f64) * t3818 * t10124 + F::cast_from(0.15176003539027279786e0_f64) * t6313 * t10153 - t31879 + t31881 - t31883 + t31886 - t31890 - t31894 - t31898 - t31900;
    t31901
}
