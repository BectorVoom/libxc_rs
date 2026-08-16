//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 993/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk993(t2298: f64, t26531: f64, t558: f64, t7817: f64, t797: f64, t305: f64, t38381: f64, t326: f64, t333: f64, t40575: f64, t40918: f64, t40922: f64, t40925: f64, t40930: f64, t40934: f64, t40938: f64, t40940: f64, t40944: f64, t5155: f64) -> (f64, f64) {
    let t40946 = t26531 * t2298;
    let t40948 = t7817 * t558;
    let t40949 = t797 * t40948;
    let t40951 = t305 * t38381;
    let t40953 = -0.54549323308490683456e-1_f64 * t40918 + 0.36366215538993788971e0_f64 * t40922 - 0.81823984962736025184e-1_f64 * t40925 - 0.40911992481368012593e-1_f64 * t40930 + 0.54549323308490683457e-1_f64 * t40934 - 0.11974241701863808564e0_f64 * t326 * t40575 - 0.44903406381989282115e-1_f64 * t40938 + 0.47896966807455234256e0_f64 * t5155 * t40940 * t333 + 0.2927036860455597649e0_f64 * t40944 - 0.8980681276397856423e-1_f64 * t40946 - 0.43905552906833964735e0_f64 * t40949 - 0.14635184302277988245e0_f64 * t40951;
    (t40948, t40953)
}
