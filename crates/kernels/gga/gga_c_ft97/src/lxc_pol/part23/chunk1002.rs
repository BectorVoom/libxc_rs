//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1002/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1002<F: Float>(t1127: F, t27712: F, t13411: F, t4939: F, t17818: F, t1111: F, t1113: F, t13443: F, t1412: F, t1417: F, t1701: F, t17851: F, t17987: F, t2035: F, t24307: F, t24346: F, t24390: F, t25057: F, t27494: F, t27521: F, t27711: F, t27721: F, t30794: F, t30807: F, t30812: F, t30816: F, t30825: F, t30829: F, t30833: F, t30840: F, t30843: F, t3766: F, t4957: F, t5003: F, t5016: F, t6055: F, t6763: F, t6778: F, t6784: F, t6979: F) -> (F, F, F) {
    let t30848 = t27712 * t1127;
    let t30852 = t13411 * t4939;
    let t30853 = t30852 * t17818;
    let t30856 = 4.0 * t3766 * t30794 + 0.2370952259137005195e-1 * t6763 * t1111 + 0.2370952259137005195e-1 * t6778 * t1111 + 0.28107073075534343171e-3 * t1412 * t5016 + 0.46509801892875584e-1 * t24346 * t4957 - 0.75080154872671831175e-1 * t1412 * t5003 + 0.22227677429409423704e-2 * t1417 * t1701 * t30807 - 0.44455354858818847408e-2 * t6784 * t30812 - 0.2108030480665075738e-3 * t30816 * t2035 * t6979 * t1113 + 0.47419045182740103901e-1 * t6784 * t1701 * t27494 * t1113 - 0.44455354858818847408e-2 * t13443 * t1701 * t30825 - 0.2370952259137005195e-1 * t1417 * t1701 * t30829 + 0.1054015240332537869e-3 * t17987 * t2035 * t30833 + 0.3404992446913580247e-1 * t27721 + 0.18164417702296932716e-2 * t27521 * t30840 + 0.3404992446913580247e-1 * t6055 * t30843 + 0.13519760450715832853e-3 * t17851 * t24390 + 0.88910709717637694816e-2 * t27711 * t25057 * t30848 - 0.51789017496114396277e-5 * t30853 * t24307;
    (t30852, t30853, t30856)
}
