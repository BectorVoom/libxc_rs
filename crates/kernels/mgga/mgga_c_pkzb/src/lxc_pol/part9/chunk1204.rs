//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1204/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1204<F: Float>(t411: F, t6546: F, t3199: F, t937: F, t1245: F, t6514: F, t410: F, t8309: F, t1227: F, t2421: F, t2363: F, t3246: F, t2393: F, t133: F, t19078: F, t19308: F, t22007: F, t22913: F, t22974: F, t23055: F, t2387: F, t2436: F, t2443: F, t2448: F, t3187: F, t3258: F, t3259: F, t3260: F, t3270: F, t3273: F, t394: F, t6523: F, t6558: F, t6566: F, t6583: F, t7832: F, t8508: F, t8511: F, t8512: F, t8515: F, t8519: F, t8520: F, t8529: F, t8539: F, t8542: F, t8549: F, t919: F, t943: F, t945: F) -> (F, F, F, F, F) {
    let t23398 = t411 * t6546;
    let t23412 = t937 * t3199;
    let t23416 = t6514 * t1245;
    let t23446 = t410 * t8309;
    let t23450 = t2421 * t1227;
    let t23465 = t2363 * t3246;
    let t23472 = t2393 * t3246;
    let t23475 = 0.79025390195226139182e1 * t2363 * t23412 * t3260 + 0.39512695097613069591e1 * t23416 * t6558 - 0.11853808529283920877e2 * t6523 * t8515 * t8520 + 0.19756347548806534796e1 * t8549 * t2443 + 0.39512695097613069591e1 * t8512 * t6566 + 0.11853808529283920877e2 * t6514 * t8511 * t8508 - 0.19756347548806534796e1 * t8529 * t8539 + 0.65854491829355115987e0 * t3273 * t6583 + 0.65854491829355115987e0 * t943 * t22913 * t133 * t945 + 0.13170898365871023197e1 * t3259 * t19308 + 0.11853808529283920877e2 * t6514 * t8515 * t8508 + 0.92196288561097162379e1 * t19078 * t3258 * t22007 * t22974 + 0.39512695097613069591e1 * t2363 * t23446 * t3260 - 0.19756347548806534796e1 * t2393 * t23450 * t3270 - 0.11853808529283920878e2 * t8519 * t7832 * t3187 * t2387 - 0.11853808529283920877e2 * t6523 * t8511 * t8520 + 0.39512695097613069591e1 * t2363 * t1245 * t919 * t6566 + 0.39512695097613069591e1 * t23465 * t2436 + 0.19756347548806534796e1 * t8542 * t7832 * t23055 * t394 - 0.19756347548806534796e1 * t23472 * t2448;
    (t23398, t23412, t23446, t23450, t23475)
}
