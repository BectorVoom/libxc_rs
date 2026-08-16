//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1437/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1437(t1: f64, t1162: f64, t12617: f64, t15860: f64, t15865: f64, t17919: f64, t18006: f64, t27152: f64, t27803: f64, t4356: f64, t44001: f64, t4435: f64, t4464: f64, t45584: f64, t45694: f64, t45719: f64, t4573: f64, t5324: f64, t5329: f64, t54716: f64, t55067: f64, t55210: f64, t55262: f64, t55265: f64, t55555: f64, t58322: f64, t59766: f64, t59822: f64, t8532: f64, t914: f64, t9169: f64, t9170: f64) -> f64 {
    let t60002 = 0.3118959061058811624e2_f64 * t55210 - 0.11721316454988582616e4_f64 * t4464 * t44001 * t54716 + 0.9291736872898228042e2_f64 * t4435 * t15865 * t17919 * t4573 * t1 + 0.75587607063262836759e5_f64 * t27803 * t55067 * t5324 + 0.71903884692229749079e5_f64 * t9169 * t5329 * t9170 * t55555 + 0.17581974682482873924e4_f64 * t4464 * t12617 * t59766 * t4356 - 0.6237918122117623248e2_f64 * t55262 - 0.99111859977581099115e3_f64 * t15860 * t18006 + 0.519826510176468604e2_f64 * t55265 - 0.18583473745796456084e3_f64 * t4435 * t15865 * t27152 * t59822 - 0.13909058383662012568e1_f64 * t1162 * t914 * t8532 * t58322 + 0.80782942410710002747e1_f64 * t45584 + 0.1343485452223045261e-1_f64 * t45694 - 0.67174272611152263053e-2_f64 * t45719;
    t60002
}
