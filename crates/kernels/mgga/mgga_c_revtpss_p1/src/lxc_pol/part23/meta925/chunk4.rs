//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3000/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3000<F: Float>(t1042: F, t1063: F, t1068: F, t15817: F, t15850: F, t19800: F, t23834: F, t23852: F, t23886: F, t3106: F, t3188: F, t42648: F, t42716: F, t42740: F, t42745: F, t4879: F, t54148: F, t54537: F, t6302: F, t6331: F, t66547: F, t66551: F, t78785: F, t79553: F, t79559: F, t79564: F, t79575: F, t79580: F) -> F {
    let t79588 = -F::cast_from(0.57165357490759649296e-3_f64) * t79553 - F::cast_from(0.14291339372689912324e-2_f64) * t3188 * t23886 - F::cast_from(0.85748036236139473944e-3_f64) * t15850 * t6331 + F::cast_from(0.14291339372689912324e-3_f64) * t79559 * t1068 + F::cast_from(0.85748036236139473947e-3_f64) * t79564 - t54148 + F::new(5.0) / F::new(3888.0) * t42716 - F::new(5.0) / F::new(486.0) * t42740 - t42745 + F::cast_from(0.64311027177104605458e-3_f64) * t15817 * t6302 + F::cast_from(0.64311027177104605458e-3_f64) * t4879 * t19800 + F::cast_from(0.68598428988911579157e-2_f64) * t42648 * t23834 - F::cast_from(0.85748036236139473947e-3_f64) * t79575 + F::cast_from(0.45732285992607719437e-2_f64) * t3106 * t23852 - F::cast_from(0.57165357490759649296e-3_f64) * t79580 - F::cast_from(0.76220476654346199062e-2_f64) * t1063 * t1042 * t54537 * t78785 + t66547 / F::new(216.0) + t66551 / F::new(54.0);
    t79588
}
