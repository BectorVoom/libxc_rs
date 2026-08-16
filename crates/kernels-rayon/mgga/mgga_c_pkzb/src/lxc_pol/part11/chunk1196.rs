//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1196/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1196(t10624: f64, t1727: f64, t10556: f64, t10572: f64, t164: f64, t1733: f64, t1734: f64, t179: f64, t20060: f64, t24135: f64, t24137: f64, t24155: f64, t24169: f64, t24171: f64, t24215: f64, t24217: f64, t24219: f64, t2660: f64, t2661: f64, t29093: f64, t29094: f64, t29210: f64, t3396: f64, t51: f64, t5279: f64, t568: f64, t590: f64, t592: f64, t612: f64, t6939: f64, t6999: f64, t8817: f64) -> f64 {
    let t29216 = t1727 * t10624;
    let t29234 = -0.77173232612525526552e-1_f64 * t20060 * t179 * t29094 + 0.60023625365297631762e-1_f64 * t24135 - 0.12004725073059526352e-1_f64 * t24137 - 0.12004725073059526352e-1_f64 * t24155 + 0.12004725073059526352e-1_f64 * t24169 - 0.68026775414003982661e-1_f64 * t24171 + 0.25724410870841842183e-2_f64 * t1733 * t179 * t10572 * t6939 - 0.12862205435420921092e-1_f64 * t5279 * t179 * t29093 * t1734 - 0.21437009059034868486e-3_f64 * t590 * t592 * t51 * t29210 * t164 + 0.10003937560882938627e-2_f64 * t29216 + 0.42874018118069736972e-2_f64 * t612 * t2660 * t51 * t10556 * t568 + 0.12862205435420921092e-1_f64 * t612 * t2660 * t6999 * t3396 + 0.12862205435420921092e-1_f64 * t612 * t2660 * t2661 * t8817 + 0.18007087609589289529e-1_f64 * t24215 - 0.24009450146119052704e-1_f64 * t24217 - 0.60023625365297631762e-2_f64 * t24219;
    t29234
}
