//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1437/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1437<F: Float>(t1: F, t1162: F, t12617: F, t15860: F, t15865: F, t17919: F, t18006: F, t27152: F, t27803: F, t4356: F, t44001: F, t4435: F, t4464: F, t45584: F, t45694: F, t45719: F, t4573: F, t5324: F, t5329: F, t54716: F, t55067: F, t55210: F, t55262: F, t55265: F, t55555: F, t58322: F, t59766: F, t59822: F, t8532: F, t914: F, t9169: F, t9170: F) -> F {
    let t60002 = F::new(0.3118959061058811624e2) * t55210 - F::new(0.11721316454988582616e4) * t4464 * t44001 * t54716 + F::new(0.9291736872898228042e2) * t4435 * t15865 * t17919 * t4573 * t1 + F::new(0.75587607063262836759e5) * t27803 * t55067 * t5324 + F::new(0.71903884692229749079e5) * t9169 * t5329 * t9170 * t55555 + F::new(0.17581974682482873924e4) * t4464 * t12617 * t59766 * t4356 - F::new(0.6237918122117623248e2) * t55262 - F::new(0.99111859977581099115e3) * t15860 * t18006 + F::new(0.519826510176468604e2) * t55265 - F::new(0.18583473745796456084e3) * t4435 * t15865 * t27152 * t59822 - F::new(0.13909058383662012568e1) * t1162 * t914 * t8532 * t58322 + F::new(0.80782942410710002747e1) * t45584 + F::new(0.1343485452223045261e-1) * t45694 - F::new(0.67174272611152263053e-2) * t45719;
    t60002
}
