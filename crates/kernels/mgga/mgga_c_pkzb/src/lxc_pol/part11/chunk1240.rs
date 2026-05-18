//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1240/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1240<F: Float>(t10786: F, t10789: F, t10873: F, t10887: F, t1107: F, t1108: F, t1955: F, t1977: F, t21146: F, t26323: F, t2848: F, t2849: F, t30255: F, t30259: F, t30261: F, t3592: F, t3604: F, t5835: F, t5838: F, t5845: F, t5903: F, t721: F, t7315: F, t7494: F, t9203: F, t9437: F, t9440: F, t9443: F, t9451: F) -> F {
    let t30498 = -t30255 - t30259 + F::new(0.10526802520742363173e2) * t7315 * t9437 - F::new(0.70178683471615754484e1) * t7494 * t9440 - F::new(0.31168546390226634765e3) * t21146 * t9443 - F::new(0.14035736694323150897e2) * t5838 * t10873 * t721 + F::new(0.10526802520742363173e2) * t1977 * t3592 * t2848 + F::new(0.6233709278045326953e3) * t5845 * t10887 * t721 - F::new(0.35089341735807877242e1) * t5903 * t10786 - F::new(0.35089341735807877242e1) * t1955 * t2849 * t3604 - F::new(0.35089341735807877242e1) * t1955 * t1108 * t9203 + F::new(0.51947577317044391277e2) * t5835 * t10789 + F::new(0.51947577317044391277e2) * t1977 * t26323 * t1107 + F::new(0.51947577317044391277e2) * t1977 * t9451 * t2848 - t30261;
    t30498
}
