//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1409/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1409<F: Float>(t2363: F, t3903: F, t2029: F, t27236: F, t10189: F, t410: F, t2393: F, t10309: F, t10323: F, t10324: F, t10334: F, t10344: F, t10349: F, t19078: F, t19090: F, t22007: F, t2433: F, t2435: F, t2436: F, t26936: F, t3270: F, t3920: F, t6455: F, t6523: F, t6566: F, t6574: F, t8436: F, t8451: F, t8520: F, t8529: F, t8539: F, t8543: F) -> (F, F, F) {
    let t28442 = t2363 * t3903;
    let t28445 = t27236 * t2029;
    let t28456 = t410 * t10189;
    let t28457 = t2393 * t28456;
    let t28476 = 0.26341796731742046394e1 * t10324 * t6566 + 0.13170898365871023197e1 * t28442 * t2436 + 0.26341796731742046394e1 * t2433 * t28445 * t2435 - 0.65854491829355115987e0 * t19090 * t10309 * t22007 * t8451 - 0.39512695097613069591e1 * t6523 * t10334 * t8520 - 0.13170898365871023197e1 * t28457 * t3270 + 0.65854491829355115987e0 * t6574 * t3920 - 0.65854491829355115987e0 * t10344 * t8539 + 0.13170898365871023197e1 * t6455 * t10323 * t8543 - 0.26341796731742046394e1 * t8529 * t10349 - 0.26341796731742046394e1 * t2393 * t26936 * t3270 + 0.92196288561097162379e1 * t19078 * t10309 * t22007 * t8436;
    (t28445, t28456, t28476)
}
