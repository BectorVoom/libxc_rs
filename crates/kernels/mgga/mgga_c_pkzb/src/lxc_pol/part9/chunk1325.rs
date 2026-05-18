//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1325/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1325<F: Float>(t2393: F, t3246: F, t1245: F, t133: F, t19078: F, t19308: F, t22007: F, t22913: F, t22974: F, t23055: F, t23412: F, t23416: F, t23446: F, t23450: F, t23465: F, t2363: F, t2387: F, t2436: F, t2443: F, t2448: F, t3187: F, t3258: F, t3259: F, t3260: F, t3270: F, t3273: F, t394: F, t6514: F, t6523: F, t6558: F, t6566: F, t6583: F, t7832: F, t8508: F, t8511: F, t8512: F, t8515: F, t8519: F, t8520: F, t8529: F, t8539: F, t8542: F, t8549: F, t919: F, t943: F, t945: F) -> F {
    let t23472 = t2393 * t3246;
    let t23475 = F::new(0.79025390195226139182e1) * t2363 * t23412 * t3260 + F::new(0.39512695097613069591e1) * t23416 * t6558 - F::new(0.11853808529283920877e2) * t6523 * t8515 * t8520 + F::new(0.19756347548806534796e1) * t8549 * t2443 + F::new(0.39512695097613069591e1) * t8512 * t6566 + F::new(0.11853808529283920877e2) * t6514 * t8511 * t8508 - F::new(0.19756347548806534796e1) * t8529 * t8539 + F::new(0.65854491829355115987e0) * t3273 * t6583 + F::new(0.65854491829355115987e0) * t943 * t22913 * t133 * t945 + F::new(0.13170898365871023197e1) * t3259 * t19308 + F::new(0.11853808529283920877e2) * t6514 * t8515 * t8508 + F::new(0.92196288561097162379e1) * t19078 * t3258 * t22007 * t22974 + F::new(0.39512695097613069591e1) * t2363 * t23446 * t3260 - F::new(0.19756347548806534796e1) * t2393 * t23450 * t3270 - F::new(0.11853808529283920878e2) * t8519 * t7832 * t3187 * t2387 - F::new(0.11853808529283920877e2) * t6523 * t8511 * t8520 + F::new(0.39512695097613069591e1) * t2363 * t1245 * t919 * t6566 + F::new(0.39512695097613069591e1) * t23465 * t2436 + F::new(0.19756347548806534796e1) * t8542 * t7832 * t23055 * t394 - F::new(0.19756347548806534796e1) * t23472 * t2448;
    t23475
}
