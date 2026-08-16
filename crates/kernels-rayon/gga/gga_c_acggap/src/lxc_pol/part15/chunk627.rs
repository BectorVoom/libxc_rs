//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 627/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk627(t1708: f64, t276: f64, t40: f64, t2961: f64, t2963: f64, t5026: f64, t2984: f64, t5032: f64, t2710: f64, t2713: f64, t2717: f64, t2737: f64, t2957: f64, t2969: f64, t2979: f64, t2983: f64, t5028: f64, t5030: f64) -> f64 {
    let t6012 = t1708 * t276;
    let t6013 = t40 * t6012;
    let t6014 = 0.17315859105681463759e2_f64 * t2961;
    let t6015 = 0.24415263074675393405e-3_f64 * t2963;
    let t6016 = 0.23392894490538584828e1_f64 * t5026;
    let t6017 = 0.5848223622634646207e0_f64 * t2984;
    let t6018 = 40.0_f64 * t5032;
    let t6019 = t6013 - t2957 - t6014 + t6015 + t2710 - t2713 - t2717 + t2737 + t6016 + t2969 + t5028 - t5030 - t2979 - t2983 - t6017 + t6018;
    t6019
}
