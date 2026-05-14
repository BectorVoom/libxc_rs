//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1115/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1115<F: Float>(t22949: F, t22954: F, t268: F, t35424: F, t6148: F, t7875: F, t11698: F, t24095: F, t1062: F, t3728: F, t6773: F, t11669: F, t2456: F, t35928: F, t35930: F, t35932: F, t35934: F, t35938: F, t35940: F, t35943: F, t35945: F, t35948: F) -> (F,) {
    let t35954 = t35424 * t268 * t22949 * t6148 * t7875 * t22954;
    let t35956 = t24095 * t11698;
    let t35959 = t1062 * t3728 * t6773;
    let t35962 = t1062 * t11669 * t2456;
    let t35964 = -0.16146599144528473358e-4 * t35928 - 0.7113065081882594864e-4 * t35930 + 0.16146599144528473358e-4 * t35932 + 0.19487085862089830731e-4 * t35934 - 0.21818671687966192679e-7 * t35938 - 0.41758041133049637282e-5 * t35940 - 0.41758041133049637282e-5 * t35943 + 0.11399142759427235359e-6 * t35945 + 0.54715885245250729722e-5 * t35948 + 0.62435555404189961692e-7 * t35954 - 0.54715885245250729722e-5 * t35956 + 0.11742981196020707897e-4 * t35959 + 0.23485962392041415794e-4 * t35962;
    (t35964,)
}
