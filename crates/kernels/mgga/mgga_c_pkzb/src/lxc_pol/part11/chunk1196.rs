//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1196/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1196<F: Float>(t10624: F, t1727: F, t10556: F, t10572: F, t164: F, t1733: F, t1734: F, t179: F, t20060: F, t24135: F, t24137: F, t24155: F, t24169: F, t24171: F, t24215: F, t24217: F, t24219: F, t2660: F, t2661: F, t29093: F, t29094: F, t29210: F, t3396: F, t51: F, t5279: F, t568: F, t590: F, t592: F, t612: F, t6939: F, t6999: F, t8817: F) -> F {
    let t29216 = t1727 * t10624;
    let t29234 = -F::new(0.77173232612525526552e-1) * t20060 * t179 * t29094 + F::new(0.60023625365297631762e-1) * t24135 - F::new(0.12004725073059526352e-1) * t24137 - F::new(0.12004725073059526352e-1) * t24155 + F::new(0.12004725073059526352e-1) * t24169 - F::new(0.68026775414003982661e-1) * t24171 + F::new(0.25724410870841842183e-2) * t1733 * t179 * t10572 * t6939 - F::new(0.12862205435420921092e-1) * t5279 * t179 * t29093 * t1734 - F::new(0.21437009059034868486e-3) * t590 * t592 * t51 * t29210 * t164 + F::new(0.10003937560882938627e-2) * t29216 + F::new(0.42874018118069736972e-2) * t612 * t2660 * t51 * t10556 * t568 + F::new(0.12862205435420921092e-1) * t612 * t2660 * t6999 * t3396 + F::new(0.12862205435420921092e-1) * t612 * t2660 * t2661 * t8817 + F::new(0.18007087609589289529e-1) * t24215 - F::new(0.24009450146119052704e-1) * t24217 - F::new(0.60023625365297631762e-2) * t24219;
    t29234
}
