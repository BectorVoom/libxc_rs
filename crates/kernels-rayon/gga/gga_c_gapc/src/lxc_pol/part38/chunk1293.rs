//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1293/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1293(t11698: f64, t24095: f64, t1062: f64, t3728: f64, t6773: f64, t11669: f64, t2456: f64, t35928: f64, t35930: f64, t35932: f64, t35934: f64, t35938: f64, t35940: f64, t35943: f64, t35945: f64, t35948: f64, t35954: f64) -> f64 {
    let t35956 = t24095 * t11698;
    let t35959 = t1062 * t3728 * t6773;
    let t35962 = t1062 * t11669 * t2456;
    let t35964 = -0.16146599144528473358e-4_f64 * t35928 - 0.7113065081882594864e-4_f64 * t35930 + 0.16146599144528473358e-4_f64 * t35932 + 0.19487085862089830731e-4_f64 * t35934 - 0.21818671687966192679e-7_f64 * t35938 - 0.41758041133049637282e-5_f64 * t35940 - 0.41758041133049637282e-5_f64 * t35943 + 0.11399142759427235359e-6_f64 * t35945 + 0.54715885245250729722e-5_f64 * t35948 + 0.62435555404189961692e-7_f64 * t35954 - 0.54715885245250729722e-5_f64 * t35956 + 0.11742981196020707897e-4_f64 * t35959 + 0.23485962392041415794e-4_f64 * t35962;
    t35964
}
