//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 780/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk780<F: Float>(t1708: F, t276: F, t40: F, t2961: F, t2963: F, t5026: F, t2984: F, t5032: F, t2710: F, t2713: F, t2717: F, t2737: F, t2957: F, t2969: F, t2979: F, t2983: F, t5028: F, t5030: F) -> (F, F, F) {
    let t6012 = t1708 * t276;
    let t6013 = t40 * t6012;
    let t6014 = F::cast_from(0.17315859105681463759e2_f64) * t2961;
    let t6015 = F::cast_from(0.24415263074675393405e-3_f64) * t2963;
    let t6016 = F::cast_from(0.23392894490538584828e1_f64) * t5026;
    let t6017 = F::cast_from(0.5848223622634646207e0_f64) * t2984;
    let t6018 = F::cast_from(40.0_f64) * t5032;
    let t6019 = t6013 - t2957 - t6014 + t6015 + t2710 - t2713 - t2717 + t2737 + t6016 + t2969 + t5028 - t5030 - t2979 - t2983 - t6017 + t6018;
    (t6012, t6013, t6019)
}
